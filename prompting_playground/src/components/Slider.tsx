import * as RadixSlider from "@radix-ui/react-slider";
import { useState } from "react";

export function Slider(props: {
  name: string;
  value: number;
  setValue: (v: number) => void;
  min?: number;
  step?: number;
  max?: number;
}) {
  const { name, value, setValue, min, step, max } = props;
  const [text, setText] = useState(value.toString());

  function editText(s: string) {
    setText(s);
    const v = parseFloat(s);
    if (!isNaN(v)) setValue(v);
  }

  function editValue(v: number) {
    setText(v.toString());
    setValue(v);
  }

  return (
    <div className="group px-2">
      <div className="flex items-center">
        <span className="flex-grow">{name}</span>
        <input
          onChange={(e) => editText(e.currentTarget.value)}
          className="w-14 rounded-md border border-transparent pr-1 text-right outline-none focus:border-teal-600 group-hover:border-neutral-300 group-hover:focus:border-teal-600"
          value={text}
        />
      </div>
      <RadixSlider.Root
        className="relative flex h-5 touch-none select-none items-center"
        value={[value]}
        onValueChange={(vs) => editValue(vs[0] ?? value)}
        max={max}
        step={step}
        min={min}
      >
        <RadixSlider.Track className="relative h-[5px] grow rounded-full bg-gray-200">
          <RadixSlider.Range className="absolute h-full rounded-full bg-neutral-300" />
        </RadixSlider.Track>
        <RadixSlider.Thumb
          className="block h-[14px] w-[14px] rounded-[10px] border-2 border-neutral-300 bg-white focus:outline-none "
          aria-label={name}
        />
      </RadixSlider.Root>
    </div>
  );
}
