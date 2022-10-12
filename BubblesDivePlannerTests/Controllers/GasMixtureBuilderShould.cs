using BubblesDivePlanner.Controllers;
using Xunit;

namespace BubblesDivePlannerTests.Controllers
{
    public class GasMixtureBuilderShould
    {
        [Theory]
        [InlineData(21, 0, 21, 0, 79)]
        [InlineData(21, 10, 21, 10, 69)]
        [InlineData(100, 0, 100, 0, 0)]
        [InlineData(0, 100, 5, 95, 0)]
        [InlineData(101, 100, 100, 0, 0)]
        [InlineData(250, 100, 100, 0, 0)]
        [InlineData(20, 101, 20, 80, 0)]
        [InlineData(100, 20, 80, 20, 0)]
        [InlineData(20, 100, 20, 80, 0)]
        [InlineData(250, 250, 100, 0, 0)]
        [InlineData(100, 100, 100, 0, 0)]
        [InlineData(0, 0, 5, 0, 95)]
        public void ConstructAGasMixture(byte oxygen, byte helium, byte expectedOxygen, byte expectedHelium, byte expectedNitrogen)
        {
            IGasMixtureBuilder gasMixtureBuilder = new GasMixtureBuilder();

            gasMixtureBuilder.WithOxygen(oxygen);
            gasMixtureBuilder.WithHelium(helium);
            var gasMixture = gasMixtureBuilder.Create();

            Assert.Equal(expectedOxygen, gasMixture.Oxygen);
            Assert.Equal(expectedHelium, gasMixture.Helium);
            Assert.Equal(expectedNitrogen, gasMixture.Nitrogen);
        }
    }
}