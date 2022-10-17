using System.Collections.Generic;
using BubblesDivePlanner.Controllers.Json;
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

        public void UpdateDiveStep(IDiveStep diveStep) => DiveStep = diveStep;

        public string Serialise()
        {
            return JsonConvert.SerializeObject(this, Formatting.Indented);
        }

        public void Deserialise(string expectedDivePlanJson)
        {
            var settings = new JsonSerializerSettings
            {
                Converters = 
                {
                    new AbstractConverter<Zhl16Buhlmann, IDiveModel>(),
                    new AbstractConverter<Cylinder, ICylinder>(),
                    new AbstractConverter<GasMixture, IGasMixture>(),
                    new AbstractConverter<DiveStep, IDiveStep>(),
                },
            };

            var divePlan = JsonConvert.DeserializeObject<DivePlan>(expectedDivePlanJson, settings);

            DiveModel = divePlan.DiveModel;
            DiveStep = divePlan.DiveStep;
            Cylinders = divePlan.Cylinders;
        }
    }
}