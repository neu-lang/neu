enum Signal { Red }

fun code(signal: Signal) {
    when (signal) {
        Signal.Red -> 0
        Signal.Red -> 1
        _ -> 2
        _ -> 3
    }
}
