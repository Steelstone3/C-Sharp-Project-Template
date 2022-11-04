using Newtonsoft.Json;

namespace BubblesDivePlanner.Models.DiveModels
{
    public class DiveProfile : IDiveProfile
    {
        public DiveProfile(byte compartmentCount)
        {
            MaxSurfacePressures = new double[compartmentCount];
            NitrogenTissuePressures = new double[compartmentCount];
            HeliumTissuePressures = new double[compartmentCount];
            TotalTissuePressures = new double[compartmentCount];
            ToleratedAmbientPressures = new double[compartmentCount];
            AValues = new double[compartmentCount];
            BValues = new double[compartmentCount];
            CompartmentLoads = new double[compartmentCount];
            DefaultValues(compartmentCount);
        }

        [JsonConstructor]
        public DiveProfile
        (
                   double[] nitrogenTissuePressures,
                   double[] heliumTissuePressures,
                   double[] totalTissuePressures,
                   double[] maxSurfacePressures,
                   double[] toleratedAmbientPressures,
                   double[] aValues,
                   double[] bValues,
                   double[] compartmentLoads,
                   double oxygenPressureAtDepth,
                   double heliumPressureAtDepth,
                   double nitrogenPressureAtDepth
        )
        {
            NitrogenTissuePressures = nitrogenTissuePressures;
            HeliumTissuePressures = heliumTissuePressures;
            TotalTissuePressures = totalTissuePressures;
            MaxSurfacePressures = maxSurfacePressures;
            ToleratedAmbientPressures = toleratedAmbientPressures;
            AValues = aValues;
            BValues = bValues;
            CompartmentLoads = compartmentLoads;
            OxygenPressureAtDepth = oxygenPressureAtDepth;
            HeliumPressureAtDepth = heliumPressureAtDepth;
            NitrogenPressureAtDepth = nitrogenPressureAtDepth;
        }

        public double[] NitrogenTissuePressures { get; private set; }
        public double[] HeliumTissuePressures { get; private set; }
        public double[] TotalTissuePressures { get; private set; }
        public double[] MaxSurfacePressures { get; private set; }
        public double[] ToleratedAmbientPressures { get; private set; }
        public double[] AValues { get; private set; }
        public double[] BValues { get; private set; }
        public double[] CompartmentLoads { get; private set; }
        public double OxygenPressureAtDepth { get; private set; }
        public double HeliumPressureAtDepth { get; private set; }
        public double NitrogenPressureAtDepth { get; private set; }

        private void DefaultValues(byte compartmentCount)
        {
            for (int compartment = 0; compartment < compartmentCount; compartment++)
            {
                NitrogenTissuePressures[compartment] = 0.79;
                TotalTissuePressures[compartment] = 0.79;
            }
        }

        public void UpdateDiveProfile(IDiveProfile diveProfile)
        {
            NitrogenTissuePressures = diveProfile.NitrogenTissuePressures;
            HeliumTissuePressures = diveProfile.HeliumTissuePressures;
            TotalTissuePressures = diveProfile.TotalTissuePressures;
            MaxSurfacePressures = diveProfile.MaxSurfacePressures;
            ToleratedAmbientPressures = diveProfile.ToleratedAmbientPressures;
            AValues = diveProfile.AValues;
            BValues = diveProfile.BValues;
            CompartmentLoads = diveProfile.CompartmentLoads;
            OxygenPressureAtDepth = diveProfile.OxygenPressureAtDepth;
            HeliumPressureAtDepth = diveProfile.HeliumPressureAtDepth;
            NitrogenPressureAtDepth = diveProfile.NitrogenPressureAtDepth;
        }

        public void UpdateGasMixtureUnderPressure(double oxygenPressureAtDepth, double heliumPressureAtDepth, double nitrogenPressureAtDepth)
        {
            OxygenPressureAtDepth = oxygenPressureAtDepth;
            HeliumPressureAtDepth = heliumPressureAtDepth;
            NitrogenPressureAtDepth = nitrogenPressureAtDepth;
        }
    }
}