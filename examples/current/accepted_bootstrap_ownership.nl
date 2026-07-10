package examples.current

public fun ownership(): Unit {
    const count: Int = 1;
    const copied: Int = count;
    const stillAvailable: Int = count;

    const text: String = "owned";
    const moved: String = text;
}
