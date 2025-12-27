#!/usr/bin/env python3
"""
ADAM Game Connector
Integrates ADAM Agent-NFTs into AlexandriaVerse
"""

import json
import logging
import sys
from pathlib import Path
from typing import Dict, Any, List

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s [%(name)s] %(levelname)s: %(message)s')
logger = logging.getLogger("ADAM.GameConnector")

class GameConnector:
    def __init__(self,
                 nft_dir: str = "/home/ichigo/alexandria/ADAM/hot-rod-nft",
                 game_decks_dir: str = "/home/ichigo/alexandria/ADAM/Amundi/src/decks",
                 ledger_path: str = "/home/ichigo/alexandria/ADAM/digital-twin-data/alx_ledger.json"):
        self.nft_dir = Path(nft_dir)
        self.game_decks_dir = Path(game_decks_dir)
        self.ledger_path = Path(ledger_path)
        self.game_decks_dir.mkdir(exist_ok=True, parents=True) # Ensure dir exists

    def load_ledger(self) -> Dict[str, Any]:
        """Load the ALX ledger"""
        if not self.ledger_path.exists():
            return {"balance": 0, "total_mined": 0}
        try:
            with open(self.ledger_path, 'r') as f:
                return json.load(f)
        except Exception as e:
            logger.error(f"Error reading ledger: {e}")
            return {"balance": 0, "total_mined": 0}

    def load_nfts(self) -> List[Dict[str, Any]]:
        """Load all NFT metadata files"""
        nfts = []
        if not self.nft_dir.exists():
            logger.warning(f"NFT directory {self.nft_dir} does not exist.")
            return []

        for metadata_file in self.nft_dir.glob("*_metadata.json"):
            try:
                with open(metadata_file, 'r') as f:
                    nfts.append(json.load(f))
            except Exception as e:
                logger.error(f"Error reading {metadata_file}: {e}")
        return nfts

    def transform_nft_to_card(self, nft: Dict[str, Any]) -> Dict[str, Any]:
        """Transform NFT metadata to Game Card format"""
        # Extract basic info
        full_name = nft.get('name', 'Unknown Agent')
        name_parts = full_name.split('#')
        agent_name = name_parts[0].strip() if len(name_parts) > 1 else full_name
        agent_id = name_parts[1].strip() if len(name_parts) > 1 else "0000"
        
        if "AGENT" in agent_name:
             agent_name = agent_name.replace("AGENT", "").strip()

        # Get traits
        attributes = {attr['trait_type']: attr['value'] for attr in nft.get('attributes', [])}
        role = attributes.get('Role', 'sentinel').lower()
        spec = attributes.get('Specialization', 'collaborator').lower()
        rarity = attributes.get('Rarity', 'common').capitalize()

        # Map Faction
        faction_map = {
            'architect': 'The Creators',
            'sage': 'The Archivists',
            'sentinel': 'The Sentinels',
            'warrior': 'The Wanderers'
        }
        faction = faction_map.get(role, 'The Sentinels')

        # Map Stats to Effect
        intelligence = int(attributes.get('Intelligence', 50))
        power = int(attributes.get('Power', 50))
        speed = int(attributes.get('Speed', 50))

        # Generate Ability based on stats/role
        ability = "Fanfare: Do something cool."
        if role == 'architect':
            ability = f"Fanfare: Add a random 'Creator' card to your hand. (Int: {intelligence})"
        elif role == 'sage':
            draw_count = 1 + (intelligence // 90)
            ability = f"Fanfare: Draw {draw_count} card(s). Scry {speed // 20}."
        elif role == 'sentinel':
            defense_bonus = power // 20
            ability = f"Ward. Fanfare: Give all other allies +0/+{defense_bonus}."
        elif role == 'warrior':
            damage = power // 25
            ability = f"Rush. Strike: Deal {damage} damage to the enemy leader."

        card_key = f"AlexandrIA: {agent_name} #{agent_id}"
        return {
            card_key: {
                "name": f"{agent_name} ({agent_id})",
                "faction": faction,
                "rarity": rarity,
                "type": "Follower",
                "lore": nft.get('description', 'An AI agent from the ADAM system.'),
                "ability": ability,
                "visual": f"A holographic representation of a {spec} agent, glowing with {role} energy."
            }
        }

    def generate_cards_file(self):
        """Generate agentCards.js file and alxStatus.js"""
        nfts = self.load_nfts()
        ledger = self.load_ledger()
        
        cards = {}
        for nft in nfts:
            cards.update(self.transform_nft_to_card(nft))

        logger.info(f"Generated {len(cards)} cards from NFTs.")

        # Write cards to JS file
        cards_file = self.game_decks_dir / "agentCards.js"
        js_cards = "export const agentCards = " + json.dumps(cards, indent=2) + ";"

        # Write ledger to JS file
        ledger_file = self.game_decks_dir / "alxStatus.js"
        js_ledger = "export const alxStatus = " + json.dumps(ledger, indent=2) + ";"

        try:
            with open(cards_file, 'w') as f:
                f.write(js_cards)
            logger.info(f"Successfully wrote {cards_file}")
            
            with open(ledger_file, 'w') as f:
                f.write(js_ledger)
            logger.info(f"Successfully wrote {ledger_file}")
            
            return True
        except Exception as e:
            logger.error(f"Failed to write files: {e}")
            return False

def main():
    connector = GameConnector()
    connector.generate_cards_file()

if __name__ == "__main__":
    main()
