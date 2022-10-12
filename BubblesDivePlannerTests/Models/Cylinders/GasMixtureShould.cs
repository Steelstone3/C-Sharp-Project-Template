using BubblesDivePlanner.Controllers;
using BubblesDivePlanner.Models.Cylinders;
using Moq;
using Xunit;

namespace BubblesDivePlannerTests.Models.Cylinders
{
    public class GasMixtureShould
    {
        [Theory]
        [InlineData(21, 0, 79)]
        [InlineData(21, 10, 69)]
        [InlineData(100, 0, 0)]
        [InlineData(0, 100, 0)]
        [InlineData(101, 100, 0)]
        [InlineData(250, 100, 0)]
        [InlineData(20, 101, 0)]
        [InlineData(100, 20, 0)]
        [InlineData(20, 100, 0)]
        [InlineData(250, 250, 0)]
        [InlineData(100, 100, 0)]
        [InlineData(0, 0, 100)]
        public void ConstructAGasMixture(byte oxygen, byte helium, byte nitrogen)
        {
            var gasMixtureBuilder = new Mock<IGasMixtureBuilder>();
            gasMixtureBuilder.Setup(gmf => gmf.WithOxygen(oxygen));
            gasMixtureBuilder.Setup(gmf => gmf.WithHelium(helium));
            gasMixtureBuilder.Setup(gmf => gmf.Create()).Returns(new GasMixture(oxygen, helium, nitrogen));
            IGasMixture gasMixture = new GasMixture(gasMixtureBuilder.Object, oxygen, helium);

            gasMixtureBuilder.VerifyAll();
        }
    }
}