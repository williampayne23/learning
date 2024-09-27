export default class Position {

    static dummy = new Position(0, 0, 0, "", "")

    col: number;
    line: number;
    file: string;
    text: string;
    nextChar: string;
    idx: number;

    constructor(idx: number, col: number, line: number, file: string, text: string) {
        this.idx = idx;
        this.col = col;
        this.line = line;
        this.file = file;
        this.text = text;
        this.nextChar = text.charAt(this.idx);
    }

    advance() {
        this.idx++;
        this.col++;
        this.nextChar = this.text.charAt(this.idx);
        if (this.nextChar == "\n") {
            this.col = 0;
            this.line++;
        }
        return this.nextChar;
    }

    copy() {
        return new Position(
            this.idx,
            this.col,
            this.line,
            this.file,
            this.text,
        );
    }
}
