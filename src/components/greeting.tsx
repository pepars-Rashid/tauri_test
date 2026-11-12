import { useEffect, useState } from "react";
import { commands, Student } from "../bindings";

export default function Greeting() {
  const [greeting, setGreeting] = useState<string>("");
  const [error, setError] = useState<string | null>(null);
  const [student, setStudent] = useState<Student>();

  useEffect(() => {
    const load = async () => {
      try {
        const result = await commands.greet("world");
        const res2 = await commands.myCommand(2);
        setStudent(res2);
        setGreeting(result);
      } catch (err) {
        setError(err instanceof Error ? err.message : String(err));
      }
    };
    load();
  }, []);

  if (error) return <div className="text-red-500">{error}</div>;
  return (
    <>
      <div className="flex flex-col">
        {greeting}
        <div className="flex flex-col">
          <div>name: {student?.name};</div>
          <div>age: {student?.age};</div>
          <div>
            name: {student?.grades[0]} {student?.grades[1]} {student?.grades[2]}
            ;
          </div>
        </div>
      </div>
    </>
  );
}
