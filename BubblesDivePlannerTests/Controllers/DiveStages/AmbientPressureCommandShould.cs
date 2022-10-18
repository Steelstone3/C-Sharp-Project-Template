using BubblesDivePlanner.Controllers.DiveStages;
using Xunit;

namespace BubblesDivePlannerTests.DiveStages
{
    public class AmbientPressureShould
    {
        [Fact]
        public void RunAmbientPressureStage()
        {
            //Arrange
            var diveProfile = TestFixture.FixtureDiveModel.DiveProfile;
            var gasMixtureModel = TestFixture.FixtureSelectedCylinder.GasMixture;
            var diveStepModel = TestFixture.FixtureDiveStep;
            IDiveStageCommand diveStage = new AmbientPressure(diveProfile, gasMixtureModel, diveStepModel);

            //Act
            diveStage.RunDiveStage();

            //Assert
            Assert.Equal(TestFixture.ExpectedPressureOxygen, diveProfile.OxygenPressureAtDepth);
            Assert.Equal(TestFixture.ExpectedPressureHelium, diveProfile.HeliumPressureAtDepth);
            Assert.Equal(TestFixture.ExpectedPressureNitrogen, diveProfile.NitrogenPressureAtDepth);
        }
    }
}