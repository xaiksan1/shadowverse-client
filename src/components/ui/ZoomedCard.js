import React from "react";
import { cardImage } from "../../decks/getCards";
import { alexandriaVerseCards } from "../../decks/alexandriaVerseCards";

export default function ZoomedCard({ hovering, name }) {
  const cardData = alexandriaVerseCards[name];

  return (
    <>
      {hovering && (
        <div
          style={{
            position: "absolute",
            top: "10%",
            height: "60%",
            zIndex: 100,
            display: "flex",
            gap: "20px",
            alignItems: "center",
          }}
        >
          <img height={"100%"} src={cardImage(name)} alt={name} />
          {cardData && (
            <div
              style={{
                backgroundColor: "rgba(0, 31, 63, 0.85)",
                color: "#E0E0E0",
                padding: "20px",
                borderRadius: "10px",
                width: "300px",
                height: "fit-content",
                border: "2px solid #7AF6FF",
                fontFamily: "'Asul', sans-serif",
                textAlign: "left",
              }}
            >
              <h2 style={{ color: "#F9A826", margin: 0 }}>{cardData.name}</h2>
              <p style={{ fontStyle: "italic", color: "#7AF6FF" }}>
                {cardData.faction} - {cardData.type}
              </p>
              <p><strong>Ability:</strong> {cardData.ability}</p>
              <hr style={{ borderColor: "#7AF6FF", opacity: 0.5 }} />
              <p style={{ fontStyle: "italic" }}>"{cardData.lore}"</p>
            </div>
          )}
        </div>
      )}
    </>
  );
}
