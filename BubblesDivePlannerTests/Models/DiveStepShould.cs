using BubblesDivePlanner.Models;
using Xunit;

namespace Name
{
    public class DiveStepShould {
        [Fact]
        public void ConstructADiveStep() {
            byte depth = 50;
            byte time = 10;
            IDiveStep diveStep = new DiveStep(depth, time);

            Assert.Equal(depth, diveStep.Depth);
            Assert.Equal(time, diveStep.Time);
        }
    }
}