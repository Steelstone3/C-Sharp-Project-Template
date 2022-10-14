using BubblesDivePlanner.Models.DiveModels;
using NuGet.Frameworks;
using Xunit;

namespace BubblesDivePlannerTests.Models.DiveModels
{
    public class DiveProfileShould
    {
        private const byte COMPARTMENT_COUNT = 10;
        private readonly double[] expectedDefaultListValue = new double[COMPARTMENT_COUNT] { 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 };
        private readonly double expectedDefaultValue = 0.0;
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
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.TissuePressuresNitrogen.Length);
            Assert.Equal(expectedDefaultListValue, diveProfile.TissuePressuresNitrogen);
        }

        [Fact]
        public void ContainsTissuePressuresHelium()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.TissuePressuresHelium.Length);
            Assert.Equal(expectedDefaultListValue, diveProfile.TissuePressuresHelium);
        }

        [Fact]
        public void ContainsTissuePressuresTotal()
        {
            Assert.Equal(COMPARTMENT_COUNT, diveProfile.TissuePressuresTotal.Length);
            Assert.Equal(expectedDefaultListValue, diveProfile.TissuePressuresTotal);
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
        public void ContainsPressureOxygen()
        {
            Assert.Equal(expectedDefaultValue, diveProfile.PressureOxygen);
        }

        [Fact]
        public void ContainsPressureHelium()
        {
            Assert.Equal(expectedDefaultValue, diveProfile.PressureHelium);
        }

        [Fact]
        public void ContainsPressureNitrogen()
        {
            Assert.Equal(expectedDefaultValue, diveProfile.PressureNitrogen);
        }
    }
}