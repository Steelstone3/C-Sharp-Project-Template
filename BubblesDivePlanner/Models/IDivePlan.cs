using System.Collections.Generic;
using BubblesDivePlanner.Models.Cylinders;
using BubblesDivePlanner.Models.DiveModels;

namespace BubblesDivePlanner.Models
{
    public interface IDivePlan
    {
        IDiveModel DiveModel { get; }
        IDiveStep DiveStep { get; }
        IList<ICylinder> Cylinders { get; }
        void UpdateDiveStep(IDiveStep diveStep);
        string Serialise();
        void Deserialise(string expectedDivePlanJson);
    }
}