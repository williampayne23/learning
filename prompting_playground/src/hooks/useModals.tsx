import React, {
  createContext,
  useState,
  type PropsWithChildren,
  useContext,
} from "react";

type IModalContext = ReturnType<typeof useAllModals>;

const ModalContext = createContext<IModalContext | undefined>(undefined);

function useModalsContext() {
  const context = useContext(ModalContext);
  if (context == undefined) {
    throw new Error("This must be used with a Modal Porvider");
  }
  return context;
}

export const useDialog = () => {
  const modals = useModalsContext();
  return {
    dialogOpen: modals.dialogOpen,
    setDialog: modals.setDialog,
    closeDialog: modals.closeDialog,
  };
};

export function TempXicon() {
  return (
    <svg className="h-3 w-3">
      <path d="M 0 0 L 12 12 M 12 0 L 0 12" stroke="black" strokeWidth={3} />
    </svg>
  );
}

export const useGenericModal = () => {
  const [content, setContent] = useState<React.ReactNode[]>([]);

  function close() {
    setContent([]);
  }

  function openModal(content: React.ReactNode) {
    setContent((oldContent) => [content, ...oldContent]);
  }

  function popContent() {
    setContent((oldContent) => oldContent.slice(0, -1));
  }

  function popSpecificContent(i: number) {
    setContent((oldContent) =>
      oldContent.slice(0, i).concat(oldContent.slice(i + 1)),
    );
  }

  function moveContentToFront(i: number) {
    setContent((oldContent) =>
      [
        oldContent[i],
        ...oldContent.slice(0, i),
        ...oldContent.slice(i + 1),
      ].filter((x) => x !== undefined),
    );
  }

  return {
    close,
    openModal,
    popContent,
    render: (
      <div className="relative h-full">
        <button onClick={close} className="absolute right-1 top-1 z-50">
          <TempXicon />
        </button>
        <div className="h-full overflow-scroll">
          {content[content.length - 1] ?? <></>}
        </div>
      </div>
    ),
    stack: content,
    popSpecificContent,
    moveContentToFront,
  };
};

function useAllModals() {
  const dialog = useGenericModal();
  return {
    dialogOpen: dialog.stack.length > 0,
    setDialog: dialog.openModal,
    closeDialog: dialog.close,
    dialogRender: dialog.render,
  };
}

export const ModalSupplier = ({ children }: PropsWithChildren) => {
  const modals = useAllModals();

  return (
    <ModalContext.Provider value={modals}>
      {modals.dialogOpen && <Dialog>{modals.dialogRender}</Dialog>}
      {children}
    </ModalContext.Provider>
  );
};

//Will not be exposed as is will expose function which takes options and returns a promise
function Dialog({ children }: PropsWithChildren) {
  return (
    <div className="bg-bg border-secondary fixed left-[30%] top-[30%] z-50 h-[40%] w-[40%] rounded-md border">
      {children}
    </div>
  );
}
