import type openai from "openai";

export type IMessage = RecursivelyExpand<openai.Chat.ChatCompletion.Choice["message"]>;

export type IRole = IMessage["role"];

type StrictExclude<T, U> = T extends U ? (U extends T ? never : T) : T;

export type IChatModel = StrictExclude<openai.Chat.ChatCompletionCreateParams["model"], string>;

type RecursivelyExpand<T> = T extends object ? { [K in keyof T]: RecursivelyExpand<T[K]> } : T;

export type IOptions = RecursivelyExpand<
  {
    model: IChatModel;
    stop?: string[];
  } & Omit<openai.Chat.ChatCompletionCreateParamsStreaming, "messages" | "model" | "stop">
>;
