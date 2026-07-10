package examples.current

public fun nullabilityFlow() {
    val maybe: String? = null;
    var assigned: String = "missing";

    if (maybe != null) {
        assigned = maybe;
        val definite: String = maybe;
    } else {
        val fallback: String = "missing";
    }
}
