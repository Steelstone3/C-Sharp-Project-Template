using BubblesDivePlanner.Models.DiveModels;
using Xunit;

namespace BubblesDivePlannerTests.Models.DiveModels
{
    public class DiveProfileShould
    {
        private const byte COMPARTMENT_COUNT = 10;
        private const double EXPECTED_PRESSURE_AT_DEPTH = 12.1;
        private readonly double[] expectedDefaultListValue = new double[COMPARTMENT_COUNT] { 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 };
        private readonly double[] expectedDefaultListTissuePressureValue = new double[COMPARTMENT_COUNT] { 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79 };
        private readonly IDiveProfile diveProfile = new DiveProfile(COMPARTMENT_COUNT);

        [Fact]
        public void ContainsMaxSurfacePressures()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.MaxSurfacePressures.Length);
            Assert.Equal(expectedDefaultListValue, diveProfile.MaxSurfacePressures);
        }

        [Fact]
        public void ContainsTissuePressuresNitrogen()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.NitrogenTissuePressures.Length);
            Assert.Equal(expectedDefaultListTissuePressureValue, diveProfile.NitrogenTissuePressures);
        }

        [Fact]
        public void ContainsTissuePressuresHelium()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.HeliumTissuePressures.Length);
            Assert.Equal(expectedDefaultListValue, diveProfile.HeliumTissuePressures);
        }

        [Fact]
        public void ContainsTissuePressuresTotal()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.TotalTissuePressures.Length);
            Assert.Equal(expectedDefaultListTissuePressureValue, diveProfile.TotalTissuePressures);
        }

        [Fact]
        public void ContainsToleratedAmbientPressures()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.ToleratedAmbientPressures.Length);
            Assert.Equal(expectedDefaultListValue, diveProfile.ToleratedAmbientPressures);
        }

        [Fact]
        public void ContainsAValues()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.AValues.Length);
            Assert.Equal(expectedDefaultListValue, diveProfile.AValues);
        }

        [Fact]
        public void ContainsBValues()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.BValues.Length);
            Assert.Equal(expectedDefaultListValue, diveProfile.BValues);
        }

        [Fact]
        public void ContainsCompartmentLoads()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.CompartmentLoads.Length);
            Assert.Equal(expectedDefaultListValue, diveProfile.CompartmentLoads);
        }

        [Fact]
        public void ContainOxygenPressureAtDepth()
        {
            Assert.Equal(0, diveProfile.OxygenPressureAtDepth);
        }

        [Fact]
        public void ContainHeliumPressureAtDepth()
        {
            Assert.Equal(0, diveProfile.HeliumPressureAtDepth);
        }

        [Fact]
        public void ContainNitrogenPressureAtDepth()
        {
            Assert.Equal(0, diveProfile.NitrogenPressureAtDepth);
        }

        [Fact]
        public void UpdateGasMixtureUnderPressure()
        {
            diveProfile.UpdateGasMixtureUnderPressure(EXPECTED_PRESSURE_AT_DEPTH, EXPECTED_PRESSURE_AT_DEPTH, EXPECTED_PRESSURE_AT_DEPTH);

            Assert.Equal(EXPECTED_PRESSURE_AT_DEPTH, diveProfile.OxygenPressureAtDepth);
            Assert.Equal(EXPECTED_PRESSURE_AT_DEPTH, diveProfile.HeliumPressureAtDepth);
            Assert.Equal(EXPECTED_PRESSURE_AT_DEPTH, diveProfile.NitrogenPressureAtDepth);
        }
    }
}