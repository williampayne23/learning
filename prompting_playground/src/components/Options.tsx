"use client";
import { useChatContext } from "~/hooks/useChatContext";
import { type IOptions, type IChatModel } from "~/types/util";
import { SearchableList } from "./SearchableList";
import { Slider } from "./Slider";
import { type KeyboardEvent, useRef } from "react";

type KeysOfValue<T, TCondition> = Required<{
  [K in keyof T]: TCondition extends T[K] ? K : never;
}>[keyof T];

type IOptionNumberKeys = KeysOfValue<IOptions, number>;

export default function Options({ showFunctionDefs }: { showFunctionDefs: () => void }) {
  return (
    <div className="flex h-full flex-col text-sm font-light">
      <ModelSelect />
      <button onClick={showFunctionDefs}>Add Functions</button>
      <OptionSlider property="temperature" max={2} min={0} step={0.01} deflt={1} />
      <OptionSlider property="max_tokens" name="Maximum length" max={1638} min={1} step={1} deflt={256} />
      <StopSequences />
      <OptionSlider property="top_p" max={1} min={0} step={0.01} deflt={1} />
      <OptionSlider property="frequency_penalty" max={2} min={0} step={0.01} deflt={0} />
      <OptionSlider property="presence_penalty" max={2} min={0} step={0.01} deflt={0} />
    </div>
  );
}

function OptionSlider({
  property,
  max,
  min,
  step,
  name,
  deflt,
}: {
  property: IOptionNumberKeys;
  max?: number;
  min?: number;
  step?: number;
  name?: string;
  deflt?: number;
}) {
  const chatContext = useChatContext();
  const nameGuard =
    name ??
    property
      .replace(/^(.)(.*)$/, (match, firstLetter: string, restOfString: string) => {
        return firstLetter.toUpperCase() + restOfString;
      })
      .replace("_", " ");

  return (
    <div className="p-2">
      <Slider
        name={nameGuard}
        value={chatContext.options[property] ?? deflt ?? 0}
        setValue={(v) => chatContext.editOption(property, v)}
        step={step}
        max={max}
        min={min}
      />
    </div>
  );
}

function ModelSelect() {
  const models: IChatModel[] = [
    "gpt-3.5-turbo",
    "gpt-3.5-turbo-0301",
    "gpt-3.5-turbo-0613",
    "gpt-3.5-turbo-16k",
    "gpt-3.5-turbo-16k-0613",
    "gpt-4",
    "gpt-4-0314",
    "gpt-4-0613",
    "gpt-4-32k",
    "gpt-4-32k-0314",
    "gpt-4-32k-0613",
  ];
  const chatContext = useChatContext();

  return (
    <>
      <span className="pl-4 pt-2 font-light">Model</span>
      <SearchableList
        onChange={(item) => chatContext.editOption("model", item)}
        value={chatContext.options.model}
        items={models}
      />
    </>
  );
}

function StopSequences() {
  const chatContext = useChatContext();
  const inputRef = useRef<HTMLInputElement>(null);
  function appendStopSequence(s: string) {
    if (chatContext.options.stop?.includes(s)) return;
    chatContext.editOption("stop", [...(chatContext.options.stop ?? []), s]);
  }

  function popStopSequence() {
    const options = chatContext.options.stop ?? [];
    chatContext.editOption("stop", options.slice(0, -1));
  }

  function filterStopSequence(s: string) {
    chatContext.editOption("stop", chatContext.options.stop?.filter((stop) => stop !== s));
  }

  function focusInput() {
    inputRef.current?.focus();
  }

  function keyDown(e: KeyboardEvent<HTMLInputElement>) {
    if (!inputRef.current) return;
    if (e.key == "Tab") {
      e.preventDefault();
      appendStopSequence(inputRef.current.value);
      inputRef.current.value = "";
      inputRef.current.style.width = "1px";
    }
    if (e.key == "Backspace" && inputRef.current.value == "") {
      popStopSequence();
    }
  }

  function updateInputSize() {
    if (!inputRef.current) return;
    inputRef.current.style.width = "1px";
    inputRef.current.style.width = inputRef.current?.scrollWidth + "px";
  }

  return (
    <div className="flex flex-col p-4">
      <span>Stop Sequences</span>
      <span className="py-0.5 text-xs font-extralight">Enter sequence and press tab</span>
      <div
        className="flex w-full cursor-text flex-wrap rounded-md border p-1 focus-within:border-teal-600"
        onClick={focusInput}
      >
        {chatContext.options.stop?.map((o, i) => (
          <div className="mr-1 mt-1 flex h-5 justify-center rounded-sm bg-neutral-200 text-center text-[10px]" key={i}>
            <span className="m-auto px-1">{o}</span>
            <button onClick={() => filterStopSequence(o)} className="px-1 hover:bg-rose-100 hover:text-orange-700">
              x
            </button>
          </div>
        ))}
        <input
          onKeyDown={keyDown}
          onChange={updateInputSize}
          ref={inputRef}
          className="my-1 w-[10px] overflow-visible outline-none"
        />
      </div>
    </div>
  );
}
