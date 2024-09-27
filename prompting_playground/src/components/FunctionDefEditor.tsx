"use client";

import { type JSONSchema7 } from "json-schema";
import { type IChatContext } from "~/hooks/useChatContext";
import { FunctionSchema } from "./SchemaEditor";
import { IoMdAddCircle } from "react-icons/io";

export default function EditFunctions({ chatContext }: { chatContext: IChatContext }) {
  function addFunction() {
    chatContext.editOption("functions", (oldFunctions) => {
      return [
        ...(oldFunctions ?? []),
        {
          name: "",
          description: "",
          parameters: {
            type: "object",
            "": {
              type: "string",
            },
          },
        },
      ];
    });
  }

  function editFunctionName(i: number, newName: string) {
    chatContext.editOption("functions", (old) => {
      const newFunctions = [...(old ?? [])];
      const funcDef = newFunctions[i];
      if (funcDef != undefined) {
        newFunctions[i] = { ...funcDef, name: newName };
      }
      return newFunctions;
    });
  }

  function editFunctionDescription(i: number, newDescription?: string) {
    chatContext.editOption("functions", (old) => {
      const newFunctions = [...(old ?? [])];
      const funcDef = newFunctions[i];
      if (funcDef != undefined) {
        newFunctions[i] = { ...funcDef, description: newDescription };
      }
      return newFunctions;
    });
  }

  function editFunctionParameters(i: number, mutator: (old: JSONSchema7) => JSONSchema7) {
    chatContext.editOption("functions", (old) => {
      const newFunctions = [...(old ?? [])];
      const funcDef = newFunctions[i];
      if (funcDef != undefined) {
        newFunctions[i] = { ...funcDef, parameters: mutator(funcDef.parameters) as Record<string, unknown> };
      }
      return newFunctions;
    });
  }

  function removeFunction(i: number) {
    chatContext.editOption("functions", (old) => {
      const newArray = [...(old ?? [])];
      newArray.splice(i, 1);
      return newArray;
    });
  }

  const functions = chatContext.options.functions ?? [];

  return (
    <>
      {functions.map((f, i) => (
        <FunctionSchema
          key={i}
          name={f.name}
          setName={(n) => editFunctionName(i, n)}
          description={f.description}
          setDescription={(d) => editFunctionDescription(i, d)}
          schema={f.parameters}
          setSchema={(s) => editFunctionParameters(i, s)}
          remove={() => removeFunction(i)}
        />
      ))}
      <button className="flex w-full items-center justify-center py-2 transition hover:scale-110" onClick={addFunction}>
        <IoMdAddCircle />
        <span className="pl-1">Add Function</span>
      </button>
    </>
  );
}
