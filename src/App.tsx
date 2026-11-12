import { Button } from "@/components/ui/button";
import Greting from "./components/greeting";

function App() {
  return (
    <div className="flex items-center justify-center min-h-screen p-4">
      <Button variant={"link"}>Test</Button>
      <Greting />
    </div>
  );
}

export default App;
