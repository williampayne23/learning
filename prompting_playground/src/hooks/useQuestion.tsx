"use client";

import { useState } from "react";
import { useDialog } from "./useModals";

export function useQuestion() {
  const dialog = useDialog();
  function askQuestion(question: string) {
    return new Promise<string>((res, _rej) => {
      dialog.setDialog(<QuestionDialog question={question} resolution={res} />);
    }).then((result) => {
      dialog.closeDialog();
      return result;
    });
  }
  return askQuestion;
}

export function QuestionDialog({
  question,
  resolution,
}: {
  question: string;
  resolution: (value: string) => void;
}) {
  const [value, setVal] = useState("");
  return (
    <div className="flex h-full items-center justify-center">
      <div className="flex w-full flex-col p-2">
        <input
          placeholder={question}
          value={value}
          className="p-2"
          onChange={(e) => setVal(e.currentTarget.value)}
        />
        <button
          className="m-2 flex-shrink rounded-sm bg-teal-600 p-2 text-white hover:bg-teal-700"
          onClick={() => resolution(value)}
        >
          Submit
        </button>
      </div>
    </div>
  );
}
