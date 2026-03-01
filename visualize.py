import pandas as pd
import matplotlib.pyplot as plt
import sys

def visualize():
    try:
        df = pd.read_csv('simulation_results.csv')
    except FileNotFoundError:
        print("Error: simulation_results.csv not found.")
        return

    plt.figure(figsize=(12, 6))
    plt.plot(df['timestamp'], df['exchange_rate'], marker='o', linestyle='-', markersize=2, alpha=0.7, color='#00aaff')
    
    plt.title('Club Pair Exchange Rate Simulation (Bayern vs Man City)', fontsize=14, fontweight='bold', color='#333333')
    plt.xlabel('Timestamp (seconds)', fontsize=12)
    plt.ylabel('Exchange Rate (V_base / V_quote)', fontsize=12)
    plt.grid(True, linestyle='--', alpha=0.6)
    
    # Premium styling
    plt.gca().set_facecolor('#f9f9f9')
    plt.gca().spines['top'].set_visible(False)
    plt.gca().spines['right'].set_visible(False)
     
    # Annotate start and end
    plt.annotate(f'Start: {df["exchange_rate"].iloc[0]:.2f}', 
                 xy=(df['timestamp'].iloc[0], df['exchange_rate'].iloc[0]),
                 xytext=(10, 10), textcoords='offset points', arrowprops=dict(arrowstyle='->', color='gray'))
    
    plt.annotate(f'End: {df["exchange_rate"].iloc[-1]:.2f}', 
                 xy=(df['timestamp'].iloc[-1], df['exchange_rate'].iloc[-1]),
                 xytext=(-40, 10), textcoords='offset points', arrowprops=dict(arrowstyle='->', color='gray'))

    plt.tight_layout()
    plt.savefig('simulation_chart.png', dpi=300)
    print("Chart saved as simulation_chart.png")

if __name__ == "__main__":
    visualize()
    sys.exit(0)
