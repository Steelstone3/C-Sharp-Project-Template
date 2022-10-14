using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Cryptography.X509Certificates;
using BubblesDivePlanner.Models.Cylinders;
using BubblesDivePlanner.Models.DiveModels;
using Newtonsoft.Json;

namespace BubblesDivePlanner.Models
{
    public class DivePlan : IDivePlan
    {
        public DivePlan(IDiveModel diveModel, IList<ICylinder> cylinders)
        {
            DiveModel = diveModel;
            Cylinders = cylinders;
        }

        public IDiveModel DiveModel { get; private set; }
        public IList<ICylinder> Cylinders { get; private set; }
        public IDiveStep DiveStep { get; private set; }

        public void UpdateDiveStep(IDiveStep diveStep)
        {
            DiveStep = diveStep;
        }

        public string Serialise()
        {
            return JsonConvert.SerializeObject(this, Formatting.Indented);
        }

        public void Deserialise(string expectedDivePlanJson)
        {
            var divePlan = JsonConvert.DeserializeObject<DivePlan>(expectedDivePlanJson);

            DiveModel = divePlan.DiveModel;
            DiveStep = divePlan.DiveStep;
            Cylinders = divePlan.Cylinders;
        }
    }
}