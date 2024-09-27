import NullValue from "./values/nullValue.ts";
import Value from "./values/value.ts";

export default class Context {
    name: string;
    parent?: Context;
    values: Record<string, Value>;

    constructor(name: string, parent?: Context) {
        this.name = name;
        this.parent = parent;
        this.values = {"": new NullValue()};
    }

    has(name: string): boolean{
        return name in this.values
    }

    set(name: string, value: Value) {
        if(!this.has(name) && this.parent?.has(name)){
            this.parent.set(name, value);
            return
        }
        this.values[name] = value;
    }

    get(name: string): Value {
        if (name in this.values) return this.values[name];
        if(this.parent) return this.parent.get(name);
        throw Error;
    }

    copy(): Context {
        const newParent = this.parent?.copy()
        const newContext = new Context(this.name, newParent);
        Object.getOwnPropertyNames(this.values).forEach((prop) => {
            newContext.set(prop, this.get(prop).copy())
        })
        return newContext;
    }
}
