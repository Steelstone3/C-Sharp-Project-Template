using BubblesDivePlanner.Controllers;
using BubblesDivePlanner.Models.Cylinders;
using Moq;
using Xunit;

namespace BubblesDivePlannerTests.Models.Cylinders
{
    public class GasMixtureShould
    {
        [Fact]
        public void ConstructAGasMixture()
        {
            byte oxygen = 21;
            byte helium = 10;

            var dummyGasMixture = new Mock<IGasMixture>();
            var stubGasMixtureBuilder = new Mock<IGasMixtureBuilder>();
            stubGasMixtureBuilder.Setup(gmf => gmf.WithOxygen(oxygen));
            stubGasMixtureBuilder.Setup(gmf => gmf.WithHelium(helium));
            stubGasMixtureBuilder.Setup(gmf => gmf.Create()).Returns(dummyGasMixture.Object);
            IGasMixture gasMixture = new GasMixture(stubGasMixtureBuilder.Object, oxygen, helium);

            stubGasMixtureBuilder.VerifyAll();
        }
    }
}