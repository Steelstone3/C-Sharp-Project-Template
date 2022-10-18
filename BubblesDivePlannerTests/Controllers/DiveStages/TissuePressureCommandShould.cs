using BubblesDivePlanner.DiveStages;
using Xunit;

namespace BubblesDivePlannerTests.DiveStages
{
    public class TissuePressureShould
    {
        [Fact]
        public void RunTissuePressureStage()
        {
            //Arrange
            var diveModel = TestFixture.FixtureDiveModel;
            var diveStep = TestFixture.FixtureDiveStep;
            diveModel.DiveProfile.OxygenPressureAtDepth = TestFixture.ExpectedPressureOxygen;
            diveModel.DiveProfile.HeliumPressureAtDepth = TestFixture.ExpectedPressureHelium;
            diveModel.DiveProfile.NitrogenPressureAtDepth = TestFixture.ExpectedPressureNitrogen;
            var diveStage = new TissuePressure(diveModel, diveStep);

            //Act
            diveStage.RunDiveStage();

            //Assert
            Assert.Equal(TestFixture.ExpectedNitrogenTissuePressures, diveModel.DiveProfile.NitrogenTissuePressures);
            Assert.Equal(TestFixture.ExpectedHeliumTissuePressures, diveModel.DiveProfile.HeliumTissuePressures);
            Assert.Equal(TestFixture.ExpectedTotalTissuePressures, diveModel.DiveProfile.TotalTissuePressures);
        }
    }
}