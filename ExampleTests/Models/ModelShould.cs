using Example.Models;
using Xunit;

namespace BubblesDivePlannerTests.Models
{
    public class ModelShould
    {
        [Fact]
        public void HaveAThing()
        {
            // Given
            IModel model = new Model();

            // Then
            Assert.Equal("The thing", model.Thing);
        }
    }
}