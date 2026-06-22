#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

struct Event {
    double x, y1, y2;
    int type; // 1 for left edge, -1 for right edge
    bool operator<(const Event& other) const {
        return x < other.x;
    }
};

int main() {
    int N;
    cin >> N;
    
    vector<Event> events;
    vector<double> y_coords;
    
    for(int i = 0; i < N; ++i) {
        double x1, y1, x2, y2;
        cin >> x1 >> y1 >> x2 >> y2;
        events.push_back({x1, y1, y2, 1});
        events.push_back({x2, y1, y2, -1});
        y_coords.push_back(y1);
        y_coords.push_back(y2);
    }
    
    // Sort and remove duplicates of y-coordinates
    sort(y_coords.begin(), y_coords.end());
    y_coords.erase(unique(y_coords.begin(), y_coords.end()), y_coords.end());
    
    // Sort events by x-coordinate
    sort(events.begin(), events.end());
    
    double total_area = 0;
    vector<int> count(y_coords.size(), 0);
    double last_x = events[0].x;
    
    for(const auto& event : events) {
        double current_x = event.x;
        double width = current_x - last_x;
        
        // Calculate active height
        double active_height = 0;
        double prev_y = -1;
        for(size_t i = 0; i < y_coords.size(); ++i) {
            if(count[i] > 0) {
                if(prev_y != -1) {
                    active_height += y_coords[i] - prev_y;
                }
            }
            prev_y = y_coords[i];
        }
        
        total_area += width * active_height;
        
        // Update the counts for y-intervals
        int y1_idx = distance(y_coords.begin(), lower_bound(y_coords.begin(), y_coords.end(), event.y1));
        int y2_idx = distance(y_coords.begin(), lower_bound(y_coords.begin(), y_coords.end(), event.y2));
        
        for(int i = y1_idx; i < y2_idx; ++i) {
            count[i] += event.type;
        }
        
        last_x = current_x;
    }
    
    cout << total_area << endl;
    return 0;
}