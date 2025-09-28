#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod alexandria_cards {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    /// Token ID type
    pub type TokenId = u32;

    /// Card rarity levels
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum CardRarity {
        Common,
        Rare,
        Epic,
        Legendary,
    }

    /// Card types based on Shadowverse classes
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum CardClass {
        Forest,
        Sword,
        Rune,
        Dragon,
        Abyss,
        Haven,
        Neutral,
    }

    /// Card metadata structure
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct CardMetadata {
        pub name: String,
        pub description: String,
        pub image_uri: String,
        pub rarity: CardRarity,
        pub class: CardClass,
        pub attack: u32,
        pub defense: u32,
        pub cost: u32,
        pub is_evolved: bool,
        pub creator: AccountId,
    }

    /// Card ownership and marketplace listing
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct CardListing {
        pub card_id: TokenId,
        pub seller: AccountId,
        pub price: Balance,
        pub is_active: bool,
    }

    /// Card creation parameters
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct CardCreateParams {
        pub name: String,
        pub description: String,
        pub image_uri: String,
        pub rarity: CardRarity,
        pub class: CardClass,
        pub attack: u32,
        pub defense: u32,
        pub cost: u32,
    }

    /// Battle information structure
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct BattleInfo {
        pub player1: AccountId,
        pub player2: AccountId,
        pub player1_cards: Vec<TokenId>,
        pub player2_cards: Vec<TokenId>,
    }

    /// Battle result structure
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct BattleResult {
        pub winner: AccountId,
        pub loser: AccountId,
        pub cards_used: Vec<TokenId>,
        pub timestamp: u64,
    }

    /// Events
    #[ink(event)]
    pub struct CardMinted {
        #[ink(topic)]
        token_id: TokenId,
        #[ink(topic)]
        owner: AccountId,
        metadata: CardMetadata,
    }

    #[ink(event)]
    pub struct CardTransfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        token_id: TokenId,
    }

    #[ink(event)]
    pub struct CardListedForSale {
        #[ink(topic)]
        token_id: TokenId,
        #[ink(topic)]
        seller: AccountId,
        price: Balance,
    }

    #[ink(event)]
    pub struct CardSold {
        #[ink(topic)]
        token_id: TokenId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        price: Balance,
    }

    #[ink(event)]
    pub struct BattleCompleted {
        #[ink(topic)]
        battle_id: u32,
        #[ink(topic)]
        winner: AccountId,
        #[ink(topic)]
        loser: AccountId,
    }

    /// Main contract storage
    #[ink(storage)]
    pub struct AlexandriaCards {
        /// Contract owner
        owner: AccountId,
        /// Next token ID to be minted
        next_token_id: TokenId,
        /// Token owners
        token_owners: Mapping<TokenId, AccountId>,
        /// Token metadata
        token_metadata: Mapping<TokenId, CardMetadata>,
        /// User balances (number of cards owned)
        balances: Mapping<AccountId, u32>,
        /// Token approvals for transfers
        token_approvals: Mapping<TokenId, AccountId>,
        /// Operator approvals (approve all)
        operator_approvals: Mapping<(AccountId, AccountId), bool>,
        /// Marketplace listings
        marketplace_listings: Mapping<TokenId, CardListing>,
        /// Active battles
        active_battles: Mapping<u32, BattleInfo>,
        /// Battle history
        battle_results: Mapping<u32, BattleResult>,
        /// Next battle ID
        next_battle_id: u32,
        /// Card pack prices
        pack_price: Balance,
        /// Evolution fees
        evolution_fee: Balance,
        /// Marketplace fee percentage (basis points, e.g., 250 = 2.5%)
        marketplace_fee: u16,
    }

    /// Contract errors
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum ContractError {
        /// Token does not exist
        TokenNotFound,
        /// Not the token owner
        NotOwner,
        /// Not approved for transfer
        NotApproved,
        /// Insufficient balance
        InsufficientBalance,
        /// Card not for sale
        NotForSale,
        /// Already listed for sale
        AlreadyListed,
        /// Battle not found
        BattleNotFound,
        /// Not a battle participant
        NotBattleParticipant,
        /// Invalid card for evolution
        InvalidEvolution,
        /// Only contract owner can perform this action
        OnlyOwner,
    }

    impl AlexandriaCards {
        /// Constructor - initializes the NFT contract
        #[ink(constructor)]
        pub fn new(pack_price: Balance, evolution_fee: Balance, marketplace_fee: u16) -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                next_token_id: 1,
                token_owners: Default::default(),
                token_metadata: Default::default(),
                balances: Default::default(),
                token_approvals: Default::default(),
                operator_approvals: Default::default(),
                marketplace_listings: Default::default(),
                active_battles: Default::default(),
                battle_results: Default::default(),
                next_battle_id: 1,
                pack_price,
                evolution_fee,
                marketplace_fee,
            }
        }

        /// Default constructor
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(1_000_000_000_000, 100_000_000_000, 250) // 1 DOT pack price, 0.1 DOT evolution fee, 2.5% marketplace fee
        }

        /// Mint a new card (only owner)
        #[ink(message)]
        pub fn mint_card(
            &mut self,
            to: AccountId,
            params: CardCreateParams,
        ) -> Result<TokenId, ContractError> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(ContractError::OnlyOwner);
            }

            let token_id = self.next_token_id;
            let metadata = CardMetadata {
                name: params.name.clone(),
                description: params.description,
                image_uri: params.image_uri,
                rarity: params.rarity,
                class: params.class,
                attack: params.attack,
                defense: params.defense,
                cost: params.cost,
                is_evolved: false,
                creator: caller,
            };

            self.token_owners.insert(&token_id, &to);
            self.token_metadata.insert(&token_id, &metadata);
            
            let current_balance = self.balances.get(&to).unwrap_or(0);
            self.balances.insert(to, &(current_balance.saturating_add(1)));

            self.next_token_id = self.next_token_id.saturating_add(1);

            self.env().emit_event(CardMinted {
                token_id,
                owner: to,
                metadata,
            });

            self.env().emit_event(CardTransfer {
                from: None,
                to,
                token_id,
            });

            Ok(token_id)
        }

        /// Get card metadata
        #[ink(message)]
        pub fn get_card_metadata(&self, token_id: TokenId) -> Option<CardMetadata> {
            self.token_metadata.get(&token_id)
        }

        /// Get card owner
        #[ink(message)]
        pub fn owner_of(&self, token_id: TokenId) -> Option<AccountId> {
            self.token_owners.get(&token_id)
        }

        /// Get balance of account
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.balances.get(&owner).unwrap_or(0)
        }

        /// Transfer card
        #[ink(message)]
        pub fn transfer(
            &mut self,
            to: AccountId,
            token_id: TokenId,
        ) -> Result<(), ContractError> {
            let caller = self.env().caller();
            let owner = self.token_owners.get(&token_id).ok_or(ContractError::TokenNotFound)?;
            
            if caller != owner && !self.is_approved(&owner, &caller, token_id) {
                return Err(ContractError::NotApproved);
            }

            self.transfer_token_from(&owner, &to, token_id)
        }

        /// List card for sale
        #[ink(message)]
        pub fn list_for_sale(
            &mut self,
            token_id: TokenId,
            price: Balance,
        ) -> Result<(), ContractError> {
            let caller = self.env().caller();
            let owner = self.token_owners.get(&token_id).ok_or(ContractError::TokenNotFound)?;
            
            if caller != owner {
                return Err(ContractError::NotOwner);
            }

            if self.marketplace_listings.contains(&token_id) {
                return Err(ContractError::AlreadyListed);
            }

            let listing = CardListing {
                card_id: token_id,
                seller: caller,
                price,
                is_active: true,
            };

            self.marketplace_listings.insert(&token_id, &listing);

            self.env().emit_event(CardListedForSale {
                token_id,
                seller: caller,
                price,
            });

            Ok(())
        }

        /// Buy card from marketplace
        #[ink(message, payable)]
        pub fn buy_card(&mut self, token_id: TokenId) -> Result<(), ContractError> {
            let caller = self.env().caller();
            let payment = self.env().transferred_value();
            
            let listing = self.marketplace_listings.get(&token_id).ok_or(ContractError::NotForSale)?;
            
            if !listing.is_active {
                return Err(ContractError::NotForSale);
            }

            if payment < listing.price {
                return Err(ContractError::InsufficientBalance);
            }

            let seller = listing.seller;
            let price = listing.price;

            // Calculate marketplace fee
            let fee = price.saturating_mul(self.marketplace_fee as Balance).saturating_div(10000);
            let seller_amount = price.saturating_sub(fee);

            // Transfer payment to seller
            if self.env().transfer(seller, seller_amount).is_err() {
                return Err(ContractError::InsufficientBalance);
            }

            // Transfer card to buyer
            self.transfer_token_from(&seller, &caller, token_id)?;

            // Remove listing
            self.marketplace_listings.remove(&token_id);

            self.env().emit_event(CardSold {
                token_id,
                from: seller,
                to: caller,
                price,
            });

            Ok(())
        }

        /// Start a battle between two players
        #[ink(message)]
        pub fn initiate_battle(
            &mut self,
            opponent: AccountId,
            my_cards: Vec<TokenId>,
        ) -> Result<u32, ContractError> {
            let caller = self.env().caller();
            
            // Verify caller owns all the cards
            for &card_id in &my_cards {
                let owner = self.token_owners.get(&card_id).ok_or(ContractError::TokenNotFound)?;
                if owner != caller {
                    return Err(ContractError::NotOwner);
                }
            }

            let battle_id = self.next_battle_id;
            let battle_info = BattleInfo {
                player1: caller,
                player2: opponent,
                player1_cards: my_cards,
                player2_cards: Vec::<TokenId>::new(),
            };
            self.active_battles.insert(battle_id, &battle_info);
            self.next_battle_id = self.next_battle_id.saturating_add(1);

            Ok(battle_id)
        }

        /// Complete a battle and record results
        #[ink(message)]
        pub fn complete_battle(
            &mut self,
            battle_id: u32,
            winner: AccountId,
        ) -> Result<(), ContractError> {
            let caller = self.env().caller();
            
            let battle_info = self.active_battles.get(&battle_id)
                .ok_or(ContractError::BattleNotFound)?;

            // Only battle participants can complete the battle
            if caller != battle_info.player1 && caller != battle_info.player2 {
                return Err(ContractError::NotBattleParticipant);
            }

            let loser = if winner == battle_info.player1 { battle_info.player2 } else { battle_info.player1 };
            let mut all_cards = battle_info.player1_cards;
            all_cards.extend(battle_info.player2_cards);

            let result = BattleResult {
                winner,
                loser,
                cards_used: all_cards,
                timestamp: self.env().block_timestamp(),
            };

            self.battle_results.insert(&battle_id, &result);
            self.active_battles.remove(&battle_id);

            self.env().emit_event(BattleCompleted {
                battle_id,
                winner,
                loser,
            });

            Ok(())
        }

        /// Evolve a card (requires payment)
        #[ink(message, payable)]
        pub fn evolve_card(&mut self, token_id: TokenId) -> Result<(), ContractError> {
            let caller = self.env().caller();
            let payment = self.env().transferred_value();
            
            if payment < self.evolution_fee {
                return Err(ContractError::InsufficientBalance);
            }

            let owner = self.token_owners.get(&token_id).ok_or(ContractError::TokenNotFound)?;
            if caller != owner {
                return Err(ContractError::NotOwner);
            }

            let mut metadata = self.token_metadata.get(&token_id).ok_or(ContractError::TokenNotFound)?;
            
            if metadata.is_evolved {
                return Err(ContractError::InvalidEvolution);
            }

            // Enhance card stats for evolution
            metadata.is_evolved = true;
            metadata.attack = metadata.attack.saturating_add(2);
            metadata.defense = metadata.defense.saturating_add(2);
            metadata.name = metadata.name + " (Evolved)";

            self.token_metadata.insert(&token_id, &metadata);

            Ok(())
        }

        /// Get marketplace listing
        #[ink(message)]
        pub fn get_listing(&self, token_id: TokenId) -> Option<CardListing> {
            self.marketplace_listings.get(&token_id)
        }

        /// Get battle result
        #[ink(message)]
        pub fn get_battle_result(&self, battle_id: u32) -> Option<BattleResult> {
            self.battle_results.get(&battle_id)
        }

        /// Internal transfer function
        fn transfer_token_from(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            token_id: TokenId,
        ) -> Result<(), ContractError> {
            self.token_owners.insert(&token_id, to);
            
            let from_balance = self.balances.get(from).unwrap_or(0);
            self.balances.insert(from, &(from_balance.saturating_sub(1)));
            
            let to_balance = self.balances.get(to).unwrap_or(0);
            self.balances.insert(to, &(to_balance.saturating_add(1)));

            // Clear any existing approvals
            self.token_approvals.remove(&token_id);

            self.env().emit_event(CardTransfer {
                from: Some(*from),
                to: *to,
                token_id,
            });

            Ok(())
        }

        /// Check if address is approved for token
        fn is_approved(&self, owner: &AccountId, operator: &AccountId, token_id: TokenId) -> bool {
            if let Some(approved) = self.token_approvals.get(&token_id) {
                if approved == *operator {
                    return true;
                }
            }
            
            self.operator_approvals.get(&(*owner, *operator)).unwrap_or(false)
        }
    }

    /// Unit tests for the NFT card contract
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::primitives::AccountId;

        /// Test contract initialization
        #[ink::test]
        fn default_works() {
            let contract = AlexandriaCards::default();
            assert_eq!(contract.next_token_id, 1);
            assert_eq!(contract.pack_price, 1_000_000_000_000);
            assert_eq!(contract.evolution_fee, 100_000_000_000);
            assert_eq!(contract.marketplace_fee, 250);
        }

        /// Test card minting
        #[ink::test]
        fn mint_card_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = AlexandriaCards::default();
            
            let params = CardCreateParams {
                name: "Test Card".to_string(),
                description: "A test card".to_string(),
                image_uri: "https://example.com/card.png".to_string(),
                rarity: CardRarity::Common,
                class: CardClass::Forest,
                attack: 3,
                defense: 2,
                cost: 1,
            };
            let result = contract.mint_card(accounts.alice, params);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1);
            assert_eq!(contract.balance_of(accounts.alice), 1);
            assert_eq!(contract.owner_of(1), Some(accounts.alice));
        }

        /// Test card transfer
        #[ink::test]
        fn transfer_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = AlexandriaCards::default();
            
            // Mint a card to Alice
            let params = CardCreateParams {
                name: "Test Card".to_string(),
                description: "A test card".to_string(),
                image_uri: "https://example.com/card.png".to_string(),
                rarity: CardRarity::Common,
                class: CardClass::Forest,
                attack: 3,
                defense: 2,
                cost: 1,
            };
            contract.mint_card(accounts.alice, params).unwrap();

            // Change caller to Alice for transfer
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            
            // Transfer card to Bob
            let result = contract.transfer(accounts.bob, 1);
            assert!(result.is_ok());
            
            assert_eq!(contract.owner_of(1), Some(accounts.bob));
            assert_eq!(contract.balance_of(accounts.alice), 0);
            assert_eq!(contract.balance_of(accounts.bob), 1);
        }

        /// Test marketplace listing
        #[ink::test]
        fn marketplace_listing_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = AlexandriaCards::default();
            
            // Mint a card to Alice
            let params = CardCreateParams {
                name: "Test Card".to_string(),
                description: "A test card".to_string(),
                image_uri: "https://example.com/card.png".to_string(),
                rarity: CardRarity::Rare,
                class: CardClass::Sword,
                attack: 5,
                defense: 4,
                cost: 3,
            };
            contract.mint_card(accounts.alice, params).unwrap();

            // Change caller to Alice
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            
            // List card for sale
            let price = 1_000_000_000_000; // 1 DOT
            let result = contract.list_for_sale(1, price);
            assert!(result.is_ok());
            
            let listing = contract.get_listing(1);
            assert!(listing.is_some());
            let listing = listing.unwrap();
            assert_eq!(listing.seller, accounts.alice);
            assert_eq!(listing.price, price);
            assert!(listing.is_active);
        }
    }


}
