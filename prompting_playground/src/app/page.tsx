"use client";
import { useEffect, useState } from "react";
import ButtonRow from "~/components/ButtonRow";
import MessageStack from "~/components/MessageStack";
import Options from "~/components/Options";
import { GiHamburgerMenu } from "react-icons/gi";
import { GrClose } from "react-icons/gr";
import { AiOutlineClose } from "react-icons/ai";
import { useChatContext } from "~/hooks/useChatContext";
import EditFunctions from "~/components/FunctionDefEditor";

export default function HomePage() {
  const [optionVisible, setOptionVisible] = useState(false);
  const chatContext = useChatContext();

  useEffect(() => {
    function handleButtonClick(e: KeyboardEvent) {
      if (e.key == "Enter" && e.metaKey) {
        chatContext.generateNewMessageWithSettings().catch(console.error);
      }
    }

    addEventListener("keydown", handleButtonClick);

    return () => {
      removeEventListener("keydown", handleButtonClick);
    };
  }, [chatContext]);

  const [showFunctionDefs, setShowFunctionDefs] = useState(true);

  return (
    <>
      {showFunctionDefs && (
        <div className="absolute top-0 z-20 flex h-full w-full items-center justify-center">
          <div className="overflow scroll relative h-[60%] w-[80%] rounded-md border bg-white">
            <div className="flex w-full justify-end">
              <button className="right-0 p-2 py-1" onClick={() => setShowFunctionDefs(false)}>
                <GrClose className="h-4 w-4" />
              </button>
            </div>
            <EditFunctions chatContext={chatContext} />
          </div>
        </div>
      )}
      <div className="flex h-full">
        <div className="flex flex-grow flex-col">
          <div className="max-h-full flex-grow overflow-scroll">
            <MessageStack />
          </div>
          <ButtonRow />
        </div>
        <div
          className={`fixed right-0 z-10 w-56 bg-white transition duration-500 md:static ${
            optionVisible ? "" : "translate-x-56 opacity-0 md:translate-x-0 md:opacity-100"
          }`}
        >
          <Options showFunctionDefs={() => setShowFunctionDefs(true)} />
        </div>
        <button
          className={`fixed right-0 z-10 p-2 transition duration-500 md:hidden ${
            optionVisible ? "" : "translate-x-56 opacity-0"
          }`}
          onClick={() => setOptionVisible((v) => !v)}
        >
          <AiOutlineClose />
        </button>
        <button className={`fixed right-0 p-2 md:hidden `} onClick={() => setOptionVisible((v) => !v)}>
          <GiHamburgerMenu />
        </button>
      </div>
    </>
  );
}
