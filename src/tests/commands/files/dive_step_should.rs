#[cfg(test)]
mod dive_step_should {
    use crate::commands::files::dive_step::dive_step::{read_dive_step_file, upsert_dive_step_file};
    use crate::tests::test_fixtures_dive_plan::test_fixtures_dive_stage::{test_fixture_dive_step_alternative, test_fixture_dive_step_default};

    #[test]
    fn dive_step_file_io() {
        let dive_steps = vec![test_fixture_dive_step_default(), test_fixture_dive_step_alternative()];

        upsert_dive_step_file(&dive_steps).expect("integration test dive_step.json file didn't upsert");
        let actual_dive_steps = read_dive_step_file();

        assert_eq!(dive_steps[0].depth, actual_dive_steps[0].depth);
        assert_eq!(dive_steps[0].time, actual_dive_steps[0].time);
        assert_eq!(dive_steps[1].depth, actual_dive_steps[1].depth);
        assert_eq!(dive_steps[1].time, actual_dive_steps[1].time);
    }
}