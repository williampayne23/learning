import { useChatContext } from "~/hooks/useChatContext";

export default function ButtonRow() {
  const chatContext = useChatContext();
  return (
    <div className="m-2 p-2">
      <button
        disabled={chatContext.predicting}
        onClick={() => {
          chatContext.generateNewMessageWithSettings().catch(console.error);
        }}
        className="rounded-md bg-teal-600 p-2 px-4 text-white hover:bg-teal-700 disabled:bg-gray-300 disabled:text-black"
      >
        Submit
      </button>
    </div>
  );
}
