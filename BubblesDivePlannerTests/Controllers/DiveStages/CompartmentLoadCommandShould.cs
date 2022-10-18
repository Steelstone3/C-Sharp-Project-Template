using BubblesDivePlanner.Controllers.DiveStages;
using BubblesDivePlannerTests.TestFixtures;
using Xunit;

namespace BubblesDivePlannerTests.DiveStages
{
    public class CompartmentLoadCommandShould
    {
        [Fact]
        public void RunCompartmentLoadStage()
        {
            //Arrange
            var diveModel = TestFixture.FixtureDiveModel;
            diveModel.DiveProfile.TotalTissuePressures = TestFixture.ExpectedTotalTissuePressures;
            diveModel.DiveProfile.MaxSurfacePressures = TestFixture.ExpectedMaxSurfacePressures;
            var diveStage = new CompartmentLoad(diveModel);

            //Act
            diveStage.RunDiveStage();

            //Assert
            Assert.Equal(TestFixture.ExpectedCompartmentLoads, diveModel.DiveProfile.CompartmentLoads);
        }
    }
}