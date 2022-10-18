using BubblesDivePlanner.Controllers.DiveStages;
using Xunit;

namespace BubblesDivePlannerTests.Controllers.DiveStages
{
    public class CompartmentLoadShould
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