using System.Collections.Generic;

namespace BubblesDivePlanner.Models
{
    public interface IDiveStep
    {
        byte Depth { get; }
        byte Time { get; }
    }
}