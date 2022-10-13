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
            var sequence = new MockSequence();
            var dummyGasMixture = new Mock<IGasMixture>();
            var stubGasMixtureBuilder = new Mock<IGasMixtureBuilder>();
            stubGasMixtureBuilder.InSequence(sequence).Setup(gmb => gmb.WithOxygen(oxygen));
            stubGasMixtureBuilder.InSequence(sequence).Setup(gmb => gmb.WithHelium(helium));
            stubGasMixtureBuilder.InSequence(sequence).Setup(gmb => gmb.Create()).Returns(dummyGasMixture.Object);
            
            IGasMixture gasMixture = new GasMixture(stubGasMixtureBuilder.Object, oxygen, helium);

            stubGasMixtureBuilder.InSequence(sequence);
            stubGasMixtureBuilder.VerifyAll();
        }
    }
}