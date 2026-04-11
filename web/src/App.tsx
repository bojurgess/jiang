import { createSignal, createResource } from "solid-js";
import "./App.css";
import { extractPalette } from "@bojurgess/colorquant/bundler";

function App() {
    const [testImageURL, setTestImageURL] = createSignal(
        "https://i.scdn.co/image/ab67616d00001e02f1dd69d7399290cc25324706",
    );

    const fetchPalette = async (url) => {
        const bytes = await fetch(url).then((r) => r.arrayBuffer());
        return extractPalette(new Uint8Array(bytes));
    };

    const [palette] = createResource(testImageURL, fetchPalette);

    return (
        <div
            style={{
                display: "flex",
                "flex-direction": "column",
                "justify-content": "center",
                "align-content": "center",
                gap: "0.5rem",
            }}
        >
            <img
                src={testImageURL()}
                style={{
                    width: "256px",
                    height: "256px",
                    "border-radius": "10px",
                }}
            />
            <div style={{ display: "flex", gap: "10px" }}>
                {palette() &&
                    Object.entries(palette()).map(([name, color]) => (
                        <div style={{ "text-align": "center" }}>
                            <div
                                style={{
                                    width: "50px",
                                    height: "50px",
                                    "border-radius": "10px",
                                    "background-color": `rgb(${color.r}, ${color.g}, ${color.b})`,
                                }}
                            />
                            <div
                                style={{
                                    "font-size": "12px",
                                    "margin-top": "4px",
                                }}
                            >
                                {name}
                            </div>
                        </div>
                    ))}
            </div>
        </div>
    );
}

export default App;
