package examples.current

public fun nullabilityFlow() {
    val maybe: String? = null;

    if (maybe != null) {
        val definite: String = maybe;
    } else {
        val fallback: String = "missing";
    }
}
