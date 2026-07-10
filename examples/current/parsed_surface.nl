package examples.current

import examples.library as lib

public struct Box {
    public val size: Int
}

internal interface Service {
    fun run();
}

public enum State {}

public fun choose() {
    val outer = one();

    if (ready) {
        val inner = outer;
    } else {
        var other = two();
        other = outer;
    }

    return;
}
