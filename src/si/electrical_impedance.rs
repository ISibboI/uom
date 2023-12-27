//! Electrical impedance (base unit ohm, m² · kg · s⁻³ · A⁻²).
//!
//! This is typically a complex number, with the real part being the [resistance](crate::si::electrical_resistance), and the complex part being the [reactance](crate::si::electrical_reactance).

quantity! {
    /// Electrical impedance (base unit ohm, m² · kg · s⁻³ · A⁻²).
    quantity: ElectricalImpedance; "electrical impedance";
    /// Dimension of electrical impedance, L²MT⁻³I⁻² (base unit ohm, m² · kg · s⁻³ · A⁻²).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N3,     // time
        N2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ElectricalImpedanceKind);
    units {
        @yottaohm: prefix!(yotta); "YΩ", "yottaohm", "yottaohms";
        @zettaohm: prefix!(zetta); "ZΩ", "zettaohm", "zettaohms";
        @exaohm: prefix!(exa); "EΩ", "exaohm", "exaohms";
        @petaohm: prefix!(peta); "PΩ", "petaohm", "petaohms";
        @teraohm: prefix!(tera); "TΩ", "teraohm", "teraohms";
        @gigaohm: prefix!(giga); "GΩ", "gigaohm", "gigaohms";
        @megaohm: prefix!(mega); "MΩ", "megaohm", "megaohms";
        @kiloohm: prefix!(kilo); "kΩ", "kiloohm", "kiloohms";
        @hectoohm: prefix!(hecto); "hΩ", "hectoohm", "hectoohms";
        @decaohm: prefix!(deca); "daΩ", "decaohm", "decaohms";
        /// Derived unit of electrical impedance.
        @ohm: prefix!(none); "Ω", "ohm", "ohms";
        @deciohm: prefix!(deci); "dΩ", "deciohm", "deciohms";
        @centiohm: prefix!(centi); "cΩ", "centiohm", "centiohms";
        @milliohm: prefix!(milli); "mΩ", "milliohm", "milliohms";
        @microohm: prefix!(micro); "µΩ", "microohm", "microohms";
        @nanoohm: prefix!(nano); "nΩ", "nanoohm", "nanoohms";
        @picoohm: prefix!(pico); "pΩ", "picoohm", "picoohms";
        @femtoohm: prefix!(femto); "fΩ", "femtoohm", "femtoohms";
        @attoohm: prefix!(atto); "aΩ", "attoohm", "attoohms";
        @zeptoohm: prefix!(zepto); "zΩ", "zeptoohm", "zeptoohms";
        @yoctoohm: prefix!(yocto); "yΩ", "yoctoohm", "yoctoohms";

        @abohm: 1.0_E-9; "abΩ", "abohm", "abohms";
        @statohm: 8.987_552_917_115_481_E11; "statΩ", "statohm", "statohms";
    }
}
