"use client";
import type openAI from "openai";
import { useEffect, useRef } from "react";
import { useChatContext } from "~/hooks/useChatContext";
import { type IRole } from "~/types/util";

export default function Message({
  message,
  id,
}: {
  message: openAI.Chat.ChatCompletion.Choice["message"];
  id: number;
}) {
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const chatContext = useChatContext();

  useEffect(() => {
    if (textAreaRef.current) {
      textAreaRef.current.style.height = "auto";
      textAreaRef.current.style.height =
        textAreaRef.current?.scrollHeight + "px";
    }
  }, [message]);

  return (
    <>
      <div className="group flex p-2 hover:bg-slate-100">
        <div className="m-2 w-20">
          <button
            onClick={() => {
              const cycle: Record<IRole, IRole> = {
                function: "system",
                system: "user",
                user: "assistant",
                assistant: "function",
              };
              chatContext.replaceMessage(id, {
                content: message.content,
                role: cycle[message.role],
              });
            }}
            className="mt-1 rounded-sm p-2 text-sm font-semibold group-hover:bg-slate-200"
          >
            {message.role.toUpperCase()}
          </button>
        </div>
        <textarea
          ref={textAreaRef}
          value={message.content ?? ""}
          onChange={(e) =>
            chatContext.replaceMessage(id, {
              ...message,
              content: e.currentTarget.value,
            })
          }
          rows={1}
          placeholder={`Enter ${message.role == "assistant" ? "an" : "a"} ${
            message.role
          } message here`}
          className="m-2 flex-grow resize-none p-3 group-hover:bg-slate-100"
        />
        <button
          onClick={() => chatContext.removeMessage(id)}
          className="mt-4 h-6 text-lg opacity-0 group-hover:text-slate-400 group-hover:opacity-100"
        >
          <span className="hover:text-slate-900">⊖</span>
        </button>
      </div>
      <hr />
    </>
  );
}
