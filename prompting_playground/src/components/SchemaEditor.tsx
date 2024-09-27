import { type JSONSchema7Definition, type JSONSchema7, type JSONSchema7TypeName } from "json-schema";
import { IoMdAddCircle } from "react-icons/io";
import { TiDelete } from "react-icons/ti";

export function FunctionSchema(props: {
  name: string;
  setName: (name: string) => void;
  schema: JSONSchema7;
  setSchema: (mutator: (old: JSONSchema7) => JSONSchema7) => void;
  description?: string;
  setDescription: (s?: string) => void;
  remove: () => void;
}) {
  function setSchemaUpstream(mutator: (oldSchema: JSONSchema7) => JSONSchema7) {
    //Need to split out the description because openAI took a perfectly sane spec and changed it to not recurse well
    props.setSchema((oldS) => {
      const newSchema = mutator({ ...oldS, description: props.description });
      props.setDescription(newSchema.description);
      delete newSchema.description;
      return newSchema;
    });
  }

  return (
    <ObjectSchema
      name={props.name}
      setName={props.setName}
      schema={{ ...props.schema, description: props.description }}
      setSchema={setSchemaUpstream}
      fixType
      removeSchema={props.remove}
      type="Function"
    />
  );
}

type SchemaUIProps = {
  schema: JSONSchema7;
  name: string;
  setSchema: (mutator: (oldSchema: JSONSchema7) => JSONSchema7) => void;
  setName?: (name: string) => void;
  removeSchema?: () => void;
  type?: string;
  fixType?: boolean;
  fixDescription?: boolean;
};

const basicTypes: JSONSchema7TypeName[] = ["boolean", "integer", "null", "number", "string"];

function SchemaProperty(props: SchemaUIProps) {
  if (basicTypes.includes(props.schema.type as JSONSchema7TypeName)) {
    return <GenericSchema {...props} />;
  }
  if (props.schema.type == "object") {
    return <ObjectSchema {...props} />;
  }
  if (props.schema.type == "array") {
    return <ArraySchema {...props} />;
  }
}

function GenericSchema(props: SchemaUIProps) {
  if (props.schema.type == undefined || Array.isArray(props.schema.type)) throw new Error(JSON.stringify(props.schema));
  const { editType, editDescription } = schemaEditGenerator(props.setSchema);
  return (
    <div className="flex">
      {props.setName == undefined ? (
        <span className="m-1 w-48 p-2">{props.name}</span>
      ) : (
        <input
          placeholder={`${props.type ?? "Property"} name`}
          className="m-1 rounded-md border p-2 outline-none hover:border-teal-700 focus:border-teal-700"
          value={props.name}
          onChange={(e) => props.setName == undefined || props.setName(e.target.value)}
        ></input>
      )}
      {props.fixType ? null : <TypeDropdown type={props.schema.type} setType={editType} />}
      {props.fixDescription ? null : (
        <input
          className="m-1 flex-grow rounded-md border p-2 outline-none hover:border-teal-700 focus:border-teal-700"
          value={props.schema.description}
          placeholder={`${props.type ?? "Property"} description`}
          onChange={(e) => editDescription(e.target.value)}
        />
      )}
      {props.removeSchema == undefined ? null : (
        <button className="px-2 transition hover:scale-125" onClick={props.removeSchema}>
          <TiDelete className="h-5 w-5" />
        </button>
      )}
    </div>
  );
}

function ArraySchema(props: SchemaUIProps) {
  const itemsSchema = props.schema.items;
  if (itemsSchema === true || itemsSchema === false || itemsSchema == undefined || Array.isArray(itemsSchema))
    throw new Error("Items must exist on array type");
  const itemsType = itemsSchema.type;
  if (itemsType === undefined || Array.isArray(itemsType)) throw new Error("Single types only");

  const { setItemsSchema } = schemaEditGenerator(props.setSchema);

  return (
    <div className="m-2 flex flex-col rounded-sm border p-2">
      <GenericSchema {...props} />
      <SchemaProperty fixDescription schema={itemsSchema} setSchema={setItemsSchema} name={"Array Type"} />
    </div>
  );
}

function ObjectSchema(props: SchemaUIProps) {
  const { addProperty, editPropertySchema, editPropertyName, removeProperty } = schemaEditGenerator(props.setSchema);
  const properties = props.schema.properties ?? {};
  const sortedKeys = Object.keys(properties).sort((p1k, p2k) => {
    const p1 = properties[p1k];
    const p2 = properties[p2k];
    if (p1 === true || p1 === false || p2 === true || p2 === false || p1 === undefined || p2 === undefined) return 0;
    return parseFloat(p1.$id ?? "0") - parseFloat(p2.$id ?? "0");
  });

  return (
    <div className="m-2 flex flex-col rounded-sm border p-2">
      <GenericSchema {...props} />
      <span className="pl-4">
        {sortedKeys.map((key, i) => {
          const property = properties[key];
          if (property == undefined) return null;
          if (property === false || property === true) return null;
          return (
            <SchemaProperty
              removeSchema={() => removeProperty(key)}
              key={i}
              name={key}
              schema={property}
              setName={(n) => editPropertyName(key, n)}
              setSchema={(mutator) => editPropertySchema(key, mutator(property))}
            />
          );
        })}
      </span>
      <button onClick={addProperty} className="flex items-center justify-center transition hover:scale-110">
        <IoMdAddCircle />
        <span className="pl-1">Add {props.type == "Function" ? "Parameter" : "Property"}</span>
      </button>
    </div>
  );
}

function TypeDropdown({ type, setType }: { type: JSONSchema7TypeName; setType: (t: JSONSchema7TypeName) => void }) {
  return (
    <select value={type} onChange={(e) => setType(e.target.value as JSONSchema7TypeName)}>
      <option value="string">string</option>
      <option value="number">number</option>
      <option value="boolean">boolean</option>
      <option value="object">object</option>
      <option value="integer">integer</option>
      <option value="array">array</option>
      <option value="null">null</option>
    </select>
  );
}

function schemaEditGenerator(setSchema: (mutator: (oldSchema: JSONSchema7) => JSONSchema7) => void) {
  function editDescription(description: string) {
    setSchema((oldSchema) => ({ ...oldSchema, description }));
  }

  function addProperty() {
    setSchema((oldSchema) => {
      const n = Object.keys(oldSchema.properties ?? {}).length;
      return {
        ...oldSchema,
        properties: {
          ...oldSchema.properties,
          [`property${n}`]: {
            $id: n.toString(),
            description: "",
            type: "string",
          },
        },
      };
    });
  }

  function editPropertyName(oldName: string, newName: string) {
    setSchema((oldSchema) => {
      const properties = { ...oldSchema.properties };
      if (Object.keys(properties).includes(newName)) return oldSchema;
      const oldProperty = properties[oldName];
      if (oldProperty == undefined) return oldSchema;
      delete properties[oldName];
      return {
        ...oldSchema,
        properties: {
          ...properties,
          [newName]: oldProperty,
        },
      };
    });
  }

  function editPropertySchema(key: string, schema: JSONSchema7Definition) {
    setSchema((oldSchema) => ({
      ...oldSchema,
      properties: {
        ...oldSchema.properties,
        [key]: schema,
      },
    }));
  }

  function editType(type: JSONSchema7TypeName) {
    setSchema((oldSchema) => {
      if (type == "array" && oldSchema.type != "array") {
        return { ...oldSchema, type, items: { type: "string" } };
      }
      return { ...oldSchema, type };
    });
  }

  function removeProperty(name: string) {
    setSchema((oldSchema) => {
      const properties = { ...oldSchema.properties };
      delete properties[name];
      return {
        ...oldSchema,
        properties,
      };
    });
  }

  function setItemsSchema(mutator: (oldSchema: JSONSchema7) => JSONSchema7) {
    setSchema((oldSchema) => {
      return {
        ...oldSchema,
        items: mutator(oldSchema.items as JSONSchema7),
      };
    });
  }

  return {
    setItemsSchema,
    editDescription,
    editType,
    editPropertyName,
    addProperty,
    removeProperty,
    editPropertySchema,
  };
}
