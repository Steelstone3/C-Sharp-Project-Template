using Example.Controllers;
using Xunit;

namespace BubblesDivePlannerTests.Controllers.DiveStages
{
    public class ControllerShould
    {
        [Fact]
        public void RunTheThingSuccessfully()
        {
            //Arrange
            IController controller = new Controller();

            //Act
            var result = controller.RunTheThing();

            //Assert
            Assert.True(result);
        }
    }
}