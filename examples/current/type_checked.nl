package examples.current

public fun typeChecked() {
    const ready: Bool = true;
    const count: Int = 1;
    const label: String = "ok";
    const absent: Null = null;

    const source: Int = 1;
    const copy: Int = source;
    const grouped: Int = (source);

    var next: Int = count;
    next = source;
}
