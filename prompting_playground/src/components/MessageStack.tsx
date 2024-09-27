import { useChatContext } from "~/hooks/useChatContext";
import Message from "~/components/Message";

export default function MessageStack() {
  const chatContext = useChatContext();

  return (
    <div className="flex flex-col overflow-scroll px-2">
      {chatContext.messages.map((m, i) => (
        <Message key={i} id={i} message={m} />
      ))}
      <button
        onClick={chatContext.appendMessage}
        className="py-4 pl-2 text-left text-lg transition duration-100 hover:bg-slate-100"
      >
        ⊕ Add Message
      </button>
    </div>
  );
}
