import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function Greting() {
  const [greting, setGreting] = useState<string>("");

  useEffect(() => {
    invoke<string>("greet", { name: "world" })
      .then((response) => setGreting(response))
      .catch((err) => console.error("Failed to invoke:", err));
  }, []);

  return <>{greting}</>;
}
