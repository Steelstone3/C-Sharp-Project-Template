using BubblesDivePlanner.Models;
using Xunit;

namespace Name
{
    public class DiveStepShould {
        [Fact]
        public void ConstructADiveStep() {
            IDiveStep diveStep = new DiveStep();

            Assert.Equal(0, diveStep.Depth);
            Assert.Equal(0, diveStep.Time);
        }
    }
}