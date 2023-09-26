use lambdaworks_math::{
    field::element::FieldElement,
    polynomial::Polynomial,
};

#[cfg(test)]
mod tests {

    use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;

    // Some of these tests work when the finite field has order greater than 2.
    use super::*;
    const ORDER: u64 = 17;
    type F = U64PrimeField<ORDER>;
    type FE = FieldElement<F>;

    fn polynomial_a() -> Polynomial<FE> {
        Polynomial::new(&[FE::new(5), FE::new(6), FE::new(4)])
    }

    fn polynomial_b() -> Polynomial<FE> {
        Polynomial::new(&[FE::new(12), FE::new(2), FE::new(6)])
    }

    fn polynomial_a_plus_b() -> Polynomial<FE> {
        Polynomial::new(&[FE::new(0), FE::new(8), FE::new(10)])
    }

    fn polynomial_b_minus_a() -> Polynomial<FE> {
        Polynomial::new(&[FE::new(7), FE::new(13), FE::new(2)])
    }

    fn polynomial_a_times_b() -> Polynomial<FE> {
        Polynomial::new(&[FE::new(9), FE::new(14), FE::new(5), FE::new(10), FE::new(7)])
    }

    #[test]
    fn adding_a_and_b_equals_a_plus_b() {
        assert_eq!(polynomial_a() + polynomial_b(), polynomial_a_plus_b());
    }

    #[test]
    fn substracting_b_and_a_equals_b_minus_a() {
        assert_eq!(polynomial_b() - polynomial_a(), polynomial_b_minus_a());
    }

    #[test]
    fn multiplying_a_and_b_equals_a_times_b() {
        assert_eq!(polynomial_a() * polynomial_b(), polynomial_a_times_b());
    }

    #[test]
    fn division_works() {
        let p1 = Polynomial::new(&[FE::new(1), FE::new(3)]);
        let p2 = Polynomial::new(&[FE::new(1), FE::new(3)]);
        let p3 = p1.mul_with_ref(&p2);
        assert_eq!(p3 / p2, p1);
    }

    #[test]
    fn evaluate_polynomial() {
        let p = Polynomial::new(&[FE::new(5), FE::new(1), FE::new(4)]);
        assert_eq!(p.evaluate(&FE::new(2)), FE::new(6));
    }

}
