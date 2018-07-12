#[derive(PartialEq)]
pub enum EqSolution {
	Zero,
	One(f32),
	Two(f32, f32)
}

pub fn solve_quadratic_equation(a: f32, b: f32, c: f32) -> EqSolution {
	let delta = b * b - 4.0 * a * c;
	if delta < 0.0 {
		EqSolution::Zero
	} else if delta == 0.0 {
		EqSolution::One(-b/(2.0*a))
	} else {
		EqSolution::Two((-b-delta.sqrt())/(2.0*a),(-b+delta.sqrt())/(2.0*a))
	}
}

#[test]
fn test_solve_quadratic_equation()
{
	assert!(solve_quadratic_equation(1.0, -2.0, -3.0) == EqSolution::Two(-1.0, 3.0));
	assert!(solve_quadratic_equation(3.0, -6.0, 3.0) == EqSolution::One(1.0));
	assert!(solve_quadratic_equation(1.0, 1.0, 1.0) == EqSolution::Zero);
}
