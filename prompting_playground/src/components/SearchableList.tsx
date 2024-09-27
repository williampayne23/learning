'use client'

import { Combobox } from '@headlessui/react'
import { useState } from 'react'
import { BsChevronDown } from 'react-icons/bs'

export function SearchableList<T extends string[]>({
  items,
  onChange,
  value,
}: {
  items: T
  onChange: (item: T[0]) => void
  value: T[0]
}) {
  const [query, setQuery] = useState('')

  return (
    <>
      <Combobox value={value} onChange={onChange}>
        <div className="relative flex flex-col">
          <Combobox.Button className="p-2/ group mx-4 my-2 flex items-center rounded-md border ui-open:border-teal-600">
            <Combobox.Input
              value={query}
              className="flex-grow rounded-md p-2 outline-none placeholder:text-black"
              spellCheck={false}
              placeholder={value}
              onChange={(e) => setQuery(e.target.value)}
            />
            <BsChevronDown className="mr-2 w-4 opacity-50 group-hover:opacity-100" />
          </Combobox.Button>
          <Combobox.Options className="absolute left-[2%] top-14 z-10 flex max-h-96 w-[96%] flex-col overflow-scroll rounded-md border bg-white drop-shadow-sm">
            {items
              .filter((item) => item.includes(query))
              .map((item, i) => (
                <SearchableListItem key={i} name={item} />
              ))}
          </Combobox.Options>
        </div>
      </Combobox>
    </>
  )
}

function SearchableListItem({ name }: { name: string }) {
  return (
    <Combobox.Option
      as={'a'}
      className="p-2 ui-selected:bg-teal-600 ui-selected:text-white ui-not-selected:ui-active:bg-teal-50"
      value={name}
    >
      {name}
    </Combobox.Option>
  )
}
