package examples.current

public fun nullabilityFlow() {
    const maybe: String? = null;
    var assigned: String = "missing";

    if (maybe != null) {
        assigned = maybe;
        const definite: String = maybe;
    } else {
        const fallback: String = "missing";
    }
}
