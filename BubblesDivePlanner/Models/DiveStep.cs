namespace BubblesDivePlanner.Models
{
    public class DiveStep : IDiveStep
    {
        public DiveStep(byte depth, byte time)
        {
            Depth = AssignDepth(depth);
            Time = AssignTime(time);
        }

        public byte Depth { get; }
        public byte Time { get; }

        private static byte AssignDepth(byte depth)
        {
            return depth switch
            {
                > 100 => 100,
                _ => depth
            };
        }

        private static byte AssignTime(byte time)
        {
            return time switch
            {
                > 60 => 60,
                < 1 => 1,
                _ => time
            };
        }
    }
}