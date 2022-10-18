namespace BubblesDivePlanner.Models.DiveModels
{
    public interface IDiveProfile
    {
        double[] NitrogenTissuePressures { get; set; }
        double[] HeliumTissuePressures { get; set; }
        double[] TotalTissuePressures { get; set; }
        double[] MaxSurfacePressures { get; set; }
        double[] ToleratedAmbientPressures { get; set; }
        double[] AValues { get; set; }
        double[] BValues { get; set; }
        double[] CompartmentLoads { get; set; }
        double OxygenPressureAtDepth { get; set; }
        double HeliumPressureAtDepth { get; set; }
        double NitrogenPressureAtDepth { get; set; }
        void UpdateGasMixtureUnderPressure
        (
            double oxygenPressureAtDepth,
            double heliumPressureAtDepth,
            double nitrogenPressureAtDepth
        );
    }
}