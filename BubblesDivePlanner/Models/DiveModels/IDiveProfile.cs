using System.Collections.Generic;

namespace BubblesDivePlanner.Models.DiveModels
{
    public interface IDiveProfile
    {
        double[] MaxSurfacePressures { get; }
        double[] NitrogenTissuePressures { get; }
        double[] HeliumTissuePressures { get; }
        double[] TotalTissuePressures { get; }
        double[] ToleratedAmbientPressures { get; }
        double[] AValues { get; }
        double[] BValues { get; }
        double[] CompartmentLoads { get; }
        double OxygenPressureAtDepth { get; }
        double HeliumPressureAtDepth { get; }
        double NitrogenPressureAtDepth { get; }
        void UpdateGasMixtureUnderPressure(double oxygenPressureAtDepth, double heliumPressureAtDepth, double nitrogenPressureAtDepth);
    }
}