using System.Collections.Generic;
using BubblesDivePlanner.Models.Cylinders;
using BubblesDivePlanner.Models.DiveModels;

namespace BubblesDivePlanner.Models
{
    public class DivePlan : IDivePlan
    {
        public DivePlan(IDiveModel diveModel, IList<ICylinder> cylinders)
        {
            DiveModel = diveModel;
            Cylinders = cylinders;
        }

        public IDiveModel DiveModel { get; }
        public IList<ICylinder> Cylinders { get; }
        public IDiveStep DiveStep { get; private set; }

        public void UpdateDiveStep(IDiveStep diveStep)
        {
            DiveStep = diveStep;
        }
    }
}