//! Electrical susceptance (base unit siemens, m⁻² · kg⁻¹ · s³ · A²).

quantity! {
    /// Electrical susceptance (base unit siemens, m⁻² · kg⁻¹ · s³ · A²).
    quantity: ElectricalSusceptance; "electrical susceptance";
    /// Dimension of electrical susceptance, L⁻²M⁻¹T³I² (base unit siemens, m⁻² · kg⁻¹ · s³ · A²).
    dimension: ISQ<
        N2,     // length
        N1,     // mass
        P3,     // time
        P2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ElectricalSusceptanceKind);
    units {
        @yottasiemens: prefix!(yotta); "YS", "yottasiemens", "yottasiemens";
        @zettasiemens: prefix!(zetta); "ZS", "zettasiemens", "zettasiemens";
        @exasiemens: prefix!(exa); "ES", "exasiemens", "exasiemens";
        @petasiemens: prefix!(peta); "PS", "petasiemens", "petasiemens";
        @terasiemens: prefix!(tera); "TS", "terasiemens", "terasiemens";
        @gigasiemens: prefix!(giga); "GS", "gigasiemens", "gigasiemens";
        @megasiemens: prefix!(mega); "MS", "megasiemens", "megasiemens";
        @kilosiemens: prefix!(kilo); "kS", "kilosiemens", "kilosiemens";
        @hectosiemens: prefix!(hecto); "hS", "hectosiemens", "hectosiemens";
        @decasiemens: prefix!(deca); "daS", "decasiemens", "decasiemens";
        /// Derived unit of electrical susceptance.
        @siemens: prefix!(none); "S", "siemens", "siemens";
        @mho: prefix!(none); "℧", "mho", "mhos";
        @decisiemens: prefix!(deci); "dS", "decisiemens", "decisiemens";
        @centisiemens: prefix!(centi); "cS", "centisiemens", "centisiemens";
        @millisiemens: prefix!(milli); "mS", "millisiemens", "millisiemens";
        @microsiemens: prefix!(micro); "µS", "microsiemens", "microsiemens";
        @nanosiemens: prefix!(nano); "nS", "nanosiemens", "nanosiemens";
        @picosiemens: prefix!(pico); "pS", "picosiemens", "picosiemens";
        @femtosiemens: prefix!(femto); "fS", "femtosiemens", "femtosiemens";
        @attosiemens: prefix!(atto); "aS", "attosiemens", "attosiemens";
        @zeptosiemens: prefix!(zepto); "zS", "zeptosiemens", "zeptosiemens";
        @yoctosiemens: prefix!(yocto); "yS", "yoctosiemens", "yoctosiemens";

        @abmho: 1.0_E9; "abmho", "abmho", "abmhos";
        @absiemens: 1.0_E9; "abS", "abmsiemens", "abmsiemens";
        @statsiemens: 1.112_650_E-12; "statS", "statsiemens", "statsiemens";
        @statmho: 1.112_650_E-12; "statmho", "statmho", "statmhos";
    }
}
